for file in "./assets/plugins/com.github.project-flaura.bevy-rpg/icons/**/*.svg"
do
    flatpak run org.inkscape.Inkscape --export-type="png" -w 128 -h 128 $file
done