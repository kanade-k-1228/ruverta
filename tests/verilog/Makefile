TOOL := ~/oss-cad-suite/bin
SRC  := test_regmap.sv
EMU  ?=

ifeq ($(EMU),iverilog)
wave.vcd: $(SRC) tb.sv
	$(TOOL)/iverilog -g2012 -o out.vvp -DSIMU $^
	$(TOOL)/vvp out.vvp
else ($(EMU),verilator)
wave.vcd: $(SRC) tb.sv
	$(TOOL)/verilator --top-module tb --Mdir out -o simu --binary --trace -DSIMU \
		-Wno-WIDTHTRUNC -Wno-WIDTHEXPAND -Wno-TIMESCALEMOD $^
	./out/simu
endif

wave: wave.vcd
	@if [ -f wave.gtkw ]; then \
		$(TOOL)/gtkwave wave.gtkw; \
	else \
		$(TOOL)/gtkwave wave.vcd; \
	fi

clean:
	rm -rf out wave.vcd

.PHONY: wave clean