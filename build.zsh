planus rust -o bindings/rs/src/gen.rs **/*.fbs
flatc -o bindings/ts --ts --gen-object-api **/*.fbs 
flatc -o bindings/swift --swift --gen-mutable **/*.fbs 
flatc -o bindings/kt --kotlin --gen-mutable **/*.fbs 