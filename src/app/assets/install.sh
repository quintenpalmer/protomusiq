set -x

cp assets/musiqapp ~/.bin/

mkdir -p ~/.local/share/icons/hicolor/48x48/apps/
cp assets/musiqapp.png ~/.local/share/icons/hicolor/48x48/apps/

sed -e "s|%USERNAME%|$HOME|g" assets/Musiqapp.desktop > ~/.local/share/applications/Musiqapp.desktop
update-desktop-database ~/.local/share/applications/
