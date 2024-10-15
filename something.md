which can do the following

(decoding into executable)

1. Decode from binary to LIR
2. Turn LIR into HIR

(engine) 3. Execute HIR

(encoding into binary)\
4. turn HIR into LIR\
5. Turn LIR into binary\

(re-useable at encoding and decoding)\
6. Validate LIR (HIR is always made from valid state)\

(tool)\
7. Optimize binaries (performance and size)\
8. Bundler to bundle multiple binaries into one\

for now I will first make the common LIR / HIR lib\
which have the modules and traits for decoding and encoding\

so you can always convert HIR <=> LIR\
but\
decoding bin into LIR is a trait (Decode)\
encoding LIR into bin is a trait (Encode)\
and also the engine takes a HIR module and can execute that\
which can be build with a build api
