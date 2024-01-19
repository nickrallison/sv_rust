// 4.1: sillyfunction

module c401_sillyfunction(input  logic a, b, c,
                     output logic y);

  assign y = ~a & ~b & ~c | a & ~b & ~c | a & ~b &  c;
endmodule
