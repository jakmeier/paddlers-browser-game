# Tagrgets to generate PNGs from SVGs

STATIC=./static
SOURCE=./art
MENU_WIDTH=400
BUILDING_WIDTH=200
PERK_WIDTH=150
UI_BUTTON_WIDTH=200
RES_SYMBOL_WIDTH=100
DUCK_WIDTH=350

all: images

images:
	$(MAKE) $(STATIC)/gui/letters.png width=$(MENU_WIDTH)
	$(MAKE) $(STATIC)/gui/duck_shapes.png width=$(MENU_WIDTH)
	$(MAKE) $(STATIC)/gui/duck_shape.png width=$(MENU_WIDTH)
	$(MAKE) $(STATIC)/gui/duck_background_shape.png width=$(MENU_WIDTH)
	$(MAKE) $(STATIC)/gui/visitor_gate.png width=$(MENU_WIDTH)
	$(MAKE) $(STATIC)/gui/plus.png width=$(MENU_WIDTH)
	$(MAKE) $(STATIC)/gui/leaves/leaves.png width=$(UI_BUTTON_WIDTH)
	$(MAKE) $(STATIC)/gui/duties.png width=$(UI_BUTTON_WIDTH)
	$(MAKE) $(STATIC)/gui/letter.png width=$(UI_BUTTON_WIDTH)
	$(MAKE) $(STATIC)/gui/letter_hov.png width=$(UI_BUTTON_WIDTH)
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
	$(MAKE) $(STATIC)/ducks/duck_happy_yellow.png width=$(DUCK_WIDTH)
	$(MAKE) $(STATIC)/ducks/duck_happy_camo.png width=$(DUCK_WIDTH)
	$(MAKE) $(STATIC)/ducks/duck_happy_white.png width=$(DUCK_WIDTH)
	$(MAKE) $(STATIC)/ducks/duck_sad_yellow.png width=$(DUCK_WIDTH)
	$(MAKE) $(STATIC)/ducks/duck_sad_camo.png width=$(DUCK_WIDTH)
	$(MAKE) $(STATIC)/ducks/duck_sad_white.png width=$(DUCK_WIDTH)

$(STATIC)/%.png: $(SOURCE)/%.svg
	inkscape --without-gui --file $< --export-png $@ --export-width $(width)

# Generated ducks, replacing colors
HEAD=12390e
FRONT=392811
MAIN=574835
TAIL1=574836
TAIL2=262011
WING=626262;fill-opacity:1


art/ducks/duck_%_camo.svg: art/ducks/duck_%.svg
	cat $< >$@

YELLOW=dfda3b
YELLOW_D=bcb95e
GOLD=89863a
INVISIBLE=626262;fill-opacity:0
GREY=222222
GREY_L=888888

art/ducks/duck_%_yellow.svg: art/ducks/duck_%.svg
	sed 's/$(HEAD)/$(YELLOW)/;s/$(FRONT)/$(YELLOW)/;s/$(MAIN)/$(YELLOW)/;s/$(TAIL1)/$(YELLOW)/;s/$(TAIL2)/$(YELLOW)/;s/$(WING)/$(INVISIBLE)/;' $< > $@

art/ducks/duck_%_white.svg: art/ducks/duck_%.svg
	sed 's/$(HEAD)/$(GREY)/;s/$(FRONT)/$(GREY_L)/;s/$(MAIN)/$(GREY_L)/;s/$(TAIL1)/$(GREY_L)/;s/$(TAIL2)/$(GREY_L)/;s/$(WING)/$(INVISIBLE)/;' $< > $@
