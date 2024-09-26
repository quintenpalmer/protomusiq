set -x

# Advance Wars
musiqinspector games gen-desktop-file "Game Boy Advance"	"Advance Wars" >	"assets/applications/GBA Advance Wars.desktop"
musiqinspector games gen-desktop-file "Game Boy Advance"	"Advance Wars 2 - Black Hole Rising" >	"assets/applications/GBA Advance Wars 2 Black Hole Rising.desktop"

# Mario RPG Games
musiqinspector games gen-desktop-file "Super Nintendo Entertainment System"	"Super Mario RPG - Legend of the Seven Stars" >	"assets/applications/SNES Super Mario RPG.desktop"
musiqinspector games gen-desktop-file "Nintendo 64"	"Paper Mario" >	"assets/applications/N64 Paper Mario.desktop"
musiqinspector games gen-desktop-file "Game Boy Advance"	"Mario & Luigi - Superstar Saga" >	"assets/applications/GBA Mario Luigi Superstar Saga.desktop"
musiqinspector games gen-desktop-file "Nintendo DS"	"Mario & Luigi - Bowser's Inside Story" >	"assets/applications/NDS Mario Luigi Bowsers Inside Story.desktop"
musiqinspector games gen-desktop-file "Nintendo DS"	"Mario & Luigi - Partners in Time" >	"assets/applications/NDS Mario Luigi Partners in Time.desktop"
musiqinspector games gen-desktop-file "GameCube"	"Paper Mario: The Thousand-Year Door" >	"assets/applications/GameCube Paper Mario The Thousand Year Door.desktop"
musiqinspector games gen-desktop-file "Wii"	"Super Paper Mario" >	"assets/applications/Wii Super Paper Mario.desktop"

chmod a+x assets/applications/*.desktop


cp assets/applications/*.desktop ~/.local/share/applications/
update-desktop-database ~/.local/share/applications/
