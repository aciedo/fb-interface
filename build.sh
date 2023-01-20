planus rust -o rs/src/gen.rs **/*.fbs
flatc -o ts --ts --gen-object-api **/*.fbs 