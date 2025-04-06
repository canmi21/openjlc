package main

import (
	"encoding/json"
	"net/http"
	"fyne.io/fyne/v2"
	"fyne.io/fyne/v2/app"
	"fyne.io/fyne/v2/container"
	"fyne.io/fyne/v2/widget"
)

type Release struct {
	TagName string `json:"tag_name"`
}

func fetchLatestTag() string {
	resp, err := http.Get("https://api.github.com/repos/canmi21/openjlc/releases/latest")
	if err != nil {
		return "v0.0.0"
	}
	defer resp.Body.Close()
	var release Release
	json.NewDecoder(resp.Body).Decode(&release)
	return release.TagName
}

func main() {
	a := app.New()
	w := a.NewWindow("installer")
	w.Resize(fyne.NewSize(300, 150))
	
	latestTag := fetchLatestTag()
	label := widget.NewLabel("OpenJLC " + latestTag)
	checkbox := widget.NewCheck("Integrated right-click menu", nil)
	
	content := container.NewVBox(label, checkbox)
	w.SetContent(content)
	


	w.ShowAndRun()
}