package main

import (
	"encoding/json"
	"io"
	"log"
	"net/http"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"

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

func downloadFile(url, filepath string) error {
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
	_, err = io.Copy(out, resp.Body)
	return err
}

func install(release Release, logger *log.Logger) {
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
		return
	}

	tempPath := filepath.Join(os.TempDir(), filename)
	err := downloadFile(downloadURL, tempPath)
	if err != nil {
		logger.Println("Download failed: ", err)
		return
	}

	appData := os.Getenv("APPDATA")
	installPath := filepath.Join(appData, "OpenJLC", "openjlc.exe")
	os.MkdirAll(filepath.Dir(installPath), 0755)
	os.Rename(tempPath, installPath)

	pathEnv := os.Getenv("PATH")
	newPath := pathEnv + ";" + filepath.Dir(installPath)
	os.Setenv("PATH", newPath)

	cmd := "setx PATH \"" + newPath + "\""
	exec.Command("cmd", "/C", cmd).Run()

	logger.Println("Installed to: ", installPath)
}

func main() {
	a := app.New()
	w := a.NewWindow("installer")
	w.Resize(fyne.NewSize(300, 250))

	logFile, _ := os.OpenFile("install.log", os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	defer logFile.Close()
	logger := log.New(logFile, "INFO: ", log.LstdFlags)

	release := fetchLatestTag()
	var content *fyne.Container

	if release.TagName == "v0.0.0" {
		errorLabel := widget.NewLabel("Please connect to the Internet")
		content = container.NewCenter(errorLabel)
	} else {
		label := widget.NewLabel("OpenJLC " + release.TagName)
		rightClickCheck := widget.NewCheck("Integrated right-click menu", nil)
		rightClickCheck.Checked = true
		agreeCheck := widget.NewCheck("I agree to comply with the community open-source license and use it only for learning purposes", nil)
		installButton := widget.NewButton("Install", nil)
		installButton.Disable()
		cancelButton := widget.NewButton("Cancel", func() { a.Quit() })

		agreeCheck.OnChanged = func(checked bool) {
			if checked {
				installButton.Enable()
			} else {
				installButton.Disable()
			}
		}

		installButton.OnTapped = func() {
			install(release, logger)
			a.Quit()
		}

		content = container.NewVBox(
			layout.NewSpacer(),
			container.NewCenter(label),
			rightClickCheck,
			widget.NewLabel(""),
			agreeCheck,
			container.NewHBox(
				layout.NewSpacer(),
				installButton,
				cancelButton,
				layout.NewSpacer(),
			),
			layout.NewSpacer(),
		)
	}

	w.SetContent(content)
	w.ShowAndRun()
}
