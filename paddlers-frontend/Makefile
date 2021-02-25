# Tagrgets to generate PNGs from SVGs

STATIC=./static
SOURCE=./art
MENU_WIDTH=400
BUILDING_WIDTH=200
PERK_WIDTH=150
UI_BUTTON_WIDTH=200
RES_SYMBOL_WIDTH=100

all: images

images:
	$(MAKE) $(STATIC)/gui/letters.png width=$(MENU_WIDTH)
	$(MAKE) $(STATIC)/gui/duck_shapes.png width=$(MENU_WIDTH)
	$(MAKE) $(STATIC)/gui/duck_shape.png width=$(MENU_WIDTH)
	$(MAKE) $(STATIC)/gui/duck_background_shape.png width=$(MENU_WIDTH)
	$(MAKE) $(STATIC)/gui/visitor_gate.png width=$(MENU_WIDTH)
	$(MAKE) $(STATIC)/gui/plus.png width=$(MENU_WIDTH)
	$(MAKE) $(STATIC)/gui/leaves/leaves.png width=$(UI_BUTTON_WIDTH)
	$(MAKE) $(STATIC)/religion.png width=$(MENU_WIDTH)
	$(MAKE) $(STATIC)/perks/conversion.png width=$(PERK_WIDTH)
	$(MAKE) $(STATIC)/perks/invitation.png width=$(PERK_WIDTH)
	$(MAKE) $(STATIC)/perks/nest_building.png width=$(PERK_WIDTH)
	$(MAKE) $(STATIC)/perks/triple_nest_building.png width=$(PERK_WIDTH)
	$(MAKE) $(STATIC)/buildings/temple.png width=$(BUILDING_WIDTH)
	$(MAKE) $(STATIC)/buildings/nest.png width=$(BUILDING_WIDTH)
	$(MAKE) $(STATIC)/buildings/nests.png width=$(BUILDING_WIDTH)
	$(MAKE) $(STATIC)/ducks/sitting_duck.png width=$(BUILDING_WIDTH)
	$(MAKE) $(STATIC)/resources/karma.png width=$(RES_SYMBOL_WIDTH)
	$(MAKE) $(STATIC)/resources/population.png width=$(RES_SYMBOL_WIDTH)

$(STATIC)/%.png: $(SOURCE)/%.svg
	inkscape --without-gui --file $< --export-png $@ --export-width $(width)
