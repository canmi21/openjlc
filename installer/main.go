package main

import (
	"encoding/json"
	"log"
	"net/http"
	"os"

	"fyne.io/fyne/v2"
	"fyne.io/fyne/v2/app"
	"fyne.io/fyne/v2/container"
	"fyne.io/fyne/v2/layout"
	"fyne.io/fyne/v2/widget"
)

type Release struct {
	TagName string `json:"tag_name"`
}

func checkNetwork() bool {
	resp, err := http.Get("https://www.bing.com")
	if err != nil {
		return false
	}
	resp.Body.Close()
	return resp.StatusCode == 200
}

func fetchLatestTag() string {
	logFile, _ := os.OpenFile("install.log", os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	defer logFile.Close()
	logger := log.New(logFile, "INFO: ", log.LstdFlags)

	if !checkNetwork() {
		logger.Println("No network connection")
		return "v0.0.0"
	}
	logger.Println("Network connected")

	resp, err := http.Get("https://api.github.com/repos/canmi21/openjlc/releases/latest")
	if err != nil {
		logger.Println("Failed to fetch release: ", err)
		return "v0.0.0"
	}
	defer resp.Body.Close()
	var release Release
	json.NewDecoder(resp.Body).Decode(&release)
	logger.Println("Fetched tag: ", release.TagName)
	return release.TagName
}

func main() {
	a := app.New()
	w := a.NewWindow("installer")
	w.Resize(fyne.NewSize(300, 200))

	latestTag := fetchLatestTag()
	var content *fyne.Container

	if latestTag == "v0.0.0" {
		errorLabel := widget.NewLabel("Please connect to the Internet")
		content = container.NewCenter(errorLabel)
	} else {
		label := widget.NewLabel("OpenJLC " + latestTag)
		checkbox := widget.NewCheck("Integrated right-click menu", nil)
		content = container.NewVBox(
			layout.NewSpacer(),
			container.NewCenter(label),
			container.NewCenter(checkbox),
			layout.NewSpacer(),
		)
	}

	w.SetContent(content)
	w.ShowAndRun()
}
