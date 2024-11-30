#TS
## To run hot-realoding with ouput in CLI nodemon
add following in script section of package.json
`"node-dev": "nodemon --watch src --ext ts --exec \"node --no-warnings --loader ts-node/esm src/classes/class.ts\""`
in CLI
`npm run node-dev`
to dynamically run a script 
`"node-dev": "nodemon --watch src --ext ts --exec \"node --no-warnings --loader ts-node/esm\"`
in CLI
`npm run node-dev -- src/classes/class.ts`
