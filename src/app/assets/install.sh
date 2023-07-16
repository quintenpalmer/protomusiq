set -x

cp assets/musiqapp ~/.bin/

sed -e "s|%USERNAME%|$HOME|g" assets/Musiqapp.desktop > ~/.local/share/applications/Musiqapp.desktop
update-desktop-database ~/.local/share/applications/
