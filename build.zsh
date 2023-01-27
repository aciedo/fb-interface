planus rust -o dist/rs/src/gen.rs **/*.fbs
flatc -o dist/ts --ts --gen-object-api **/*.fbs 
flatc -o dist/swift --swift --gen-mutable **/*.fbs 
flatc -o dist/kt --kotlin --gen-mutable **/*.fbs 