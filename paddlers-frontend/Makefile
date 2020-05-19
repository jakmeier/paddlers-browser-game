# Tagrgets to generate PNGs from SVGs

STATIC=./static
SOURCE=./art
MENU_WIDTH=400
BUILDING_WIDTH=200

all: images

images:
	$(MAKE) $(STATIC)/gui/letters.png width=$(MENU_WIDTH)
	$(MAKE) $(STATIC)/gui/duck_shapes.png width=$(MENU_WIDTH)
	$(MAKE) $(STATIC)/buildings/nest.png width=$(BUILDING_WIDTH)
	$(MAKE) $(STATIC)/buildings/nests.png width=$(BUILDING_WIDTH)
	$(MAKE) $(STATIC)/ducks/sitting_duck.png width=$(BUILDING_WIDTH)

$(STATIC)/%.png: $(SOURCE)/%.svg
	inkscape --without-gui --file $< --export-png $@ --export-width $(width)
