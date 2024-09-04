TOOL := ~/oss-cad-suite/bin
EMU  ?=

ifeq ($(EMU),iverilog)
%.vcd: %.sv %_tb.sv
	$(TOOL)/iverilog -g2012 -o $*.vpp -DSIMU $^
	$(TOOL)/vvp $*.vpp
else ($(EMU),verilator)
%.vcd: %.sv %_tb.sv
	$(TOOL)/verilator --top-module tb --Mdir out -o simu --binary --trace -DSIMU \
		-Wno-WIDTHTRUNC -Wno-WIDTHEXPAND -Wno-TIMESCALEMOD $^
	./out/simu
endif

%: %.vcd
	@if [ -f $@$(GTKW_EXT) ]; then \
		$(TOOL)/gtkwave $@.gtkw; \
	else \
		$(TOOL)/gtkwave $@.vcd; \
	fi
