package main

import (
	"encoding/json"
	"io"
	"log"
	"net/http"
	"os"
	"path/filepath"
	"runtime"

	"golang.org/x/sys/windows/registry"

	"fyne.io/fyne/v2"
	"fyne.io/fyne/v2/app"
	"fyne.io/fyne/v2/container"
	"fyne.io/fyne/v2/layout"
	"fyne.io/fyne/v2/widget"
)

type Release struct {
	TagName string  `json:"tag_name"`
	Assets  []Asset `json:"assets"`
}

type Asset struct {
	Name               string `json:"name"`
	BrowserDownloadURL string `json:"browser_download_url"`
}

func checkNetwork() bool {
	resp, err := http.Get("https://www.bing.com")
	if err != nil {
		return false
	}
	resp.Body.Close()
	return resp.StatusCode == 200
}

func fetchLatestTag() Release {
	logFile, _ := os.OpenFile("install.log", os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	defer logFile.Close()
	logger := log.New(logFile, "INFO: ", log.LstdFlags)

	if !checkNetwork() {
		logger.Println("No network connection")
		return Release{TagName: "v0.0.0"}
	}
	logger.Println("Network connected")

	resp, err := http.Get("https://api.github.com/repos/canmi21/openjlc/releases/latest")
	if err != nil {
		logger.Println("Failed to fetch release: ", err)
		return Release{TagName: "v0.0.0"}
	}
	defer resp.Body.Close()
	var release Release
	json.NewDecoder(resp.Body).Decode(&release)
	logger.Println("Fetched tag: ", release.TagName)
	return release
}

func downloadFile(url, filepath string, progress *widget.ProgressBar) error {
	resp, err := http.Get(url)
	if err != nil {
		return err
	}
	defer resp.Body.Close()

	out, err := os.Create(filepath)
	if err != nil {
		return err
	}
	defer out.Close()

	totalSize := float64(resp.ContentLength)
	var downloadedSize float64

	buf := make([]byte, 1024)
	for {
		n, err := resp.Body.Read(buf)
		if n > 0 {
			downloadedSize += float64(n)
			progress.SetValue(downloadedSize / totalSize)
			out.Write(buf[:n])
		}
		if err == io.EOF {
			break
		}
		if err != nil {
			return err
		}
	}
	return nil
}

func install(release Release, logger *log.Logger, integrateRightClick bool, progress *widget.ProgressBar, statusLabel *widget.Label, content *fyne.Container, window fyne.Window) {
	arch := runtime.GOARCH
	var filename string
	if arch == "386" {
		filename = "openjlc-i686-pc-windows-msvc.exe"
	} else {
		filename = "openjlc-x86_64-pc-windows-msvc.exe"
	}

	var downloadURL string
	for _, asset := range release.Assets {
		if asset.Name == filename {
			downloadURL = asset.BrowserDownloadURL
			break
		}
	}

	if downloadURL == "" {
		logger.Println("No matching executable found")
		statusLabel.SetText("No matching executable found")
		return
	}

	tempPath := filepath.Join(os.TempDir(), filename)
	err := downloadFile(downloadURL, tempPath, progress)
	if err != nil {
		logger.Println("Download failed: ", err)
		statusLabel.SetText("Download failed")
		return
	}

	installDir := "C:\\Program Files\\OpenJLC"
	if _, err := os.Stat("D:\\Program Files"); err == nil {
		installDir = "D:\\Program Files\\OpenJLC"
	}

	installPath := filepath.Join(installDir, "openjlc.exe")
	os.MkdirAll(installDir, 0755)
	os.Rename(tempPath, installPath)

	k, err := registry.OpenKey(registry.LOCAL_MACHINE, `SYSTEM\CurrentControlSet\Control\Session Manager\Environment`, registry.QUERY_VALUE|registry.SET_VALUE)
	if err != nil {
		logger.Println("Failed to open registry: ", err)
		statusLabel.SetText("Failed to update registry")
		return
	}
	defer k.Close()

	pathEnv, _, err := k.GetStringValue("Path")
	if err != nil {
		logger.Println("Failed to read PATH: ", err)
		statusLabel.SetText("Failed to read PATH")
		return
	}

	newPath := pathEnv + ";" + installDir
	err = k.SetStringValue("Path", newPath)
	if err != nil {
		logger.Println("Failed to update PATH: ", err)
		statusLabel.SetText("Failed to update PATH")
		return
	}

	if integrateRightClick {
		zipKey, _, err := registry.CreateKey(registry.CLASSES_ROOT, `.zip\shell\Open with OpenJLC\command`, registry.ALL_ACCESS)
		if err != nil {
			logger.Println("Failed to create .zip shell key: ", err)
			statusLabel.SetText("Failed to create right-click integration")
			return
		}
		defer zipKey.Close()
		zipKey.SetStringValue("", "\""+installPath+"\" \"%1\"")
	}

	logger.Println("Installed to: ", installPath)
	content.Objects = []fyne.CanvasObject{
		layout.NewSpacer(),
		widget.NewLabelWithStyle("Installation Complete", fyne.TextAlignCenter, fyne.TextStyle{Bold: true}),
		widget.NewButton("Finish", func() { window.Close() }),
		layout.NewSpacer(),
	}
	content.Refresh()
}

func main() {
	a := app.New()
	w := a.NewWindow("OpenJLC Installer")
	w.Resize(fyne.NewSize(350, 150))

	logFile, _ := os.OpenFile("install.log", os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	defer logFile.Close()
	logger := log.New(logFile, "> ", log.LstdFlags)

	release := fetchLatestTag()
	var content *fyne.Container

	if release.TagName == "v0.0.0" {
		errorLabel := widget.NewLabel("No Internet connection detected")
		errorLabel.Alignment = fyne.TextAlignCenter
		content = container.NewVBox(
			layout.NewSpacer(),
			errorLabel,
			layout.NewSpacer(),
		)
	} else {
		title := widget.NewLabelWithStyle("OpenJLC "+release.TagName, fyne.TextAlignCenter, fyne.TextStyle{Bold: true})
		rightClickCheck := widget.NewCheck("Add to right-click menu for .zip", nil)
		rightClickCheck.Checked = true
		agreeCheck := widget.NewCheck("I agree to the community open-source license", nil)
		installButton := widget.NewButton("Install", nil)
		cancelButton := widget.NewButton("Cancel", func() { a.Quit() })
		progress := widget.NewProgressBar()
		statusLabel := widget.NewLabel("")

		installButton.Disable()
		agreeCheck.OnChanged = func(checked bool) {
			if checked {
				installButton.Enable()
			} else {
				installButton.Disable()
			}
		}

		installButton.OnTapped = func() {
			statusLabel.SetText("Installing...")
			progress.SetValue(0)
			content.Objects = append(content.Objects, progress)
			content.Refresh()
			go install(release, logger, rightClickCheck.Checked, progress, statusLabel, content, w)
		}

		content = container.NewVBox(
			layout.NewSpacer(),
			title,
			rightClickCheck,
			agreeCheck,
			statusLabel,
			container.NewHBox(
				layout.NewSpacer(),
				installButton,
				cancelButton,
				layout.NewSpacer(),
			),
			layout.NewSpacer(),
		)
	}

	w.SetContent(container.NewPadded(content))
	w.ShowAndRun()
}
