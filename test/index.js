const path = require("path");
const tape = require("tape");

const {
  Diorama,
  tapeExecutor,
  backwardCompatibilityMiddleware
} = require("@holochain/diorama");

process.on("unhandledRejection", error => {
  // Will print "unhandledRejection err is not defined"
  console.error("got unhandledRejection:", error);
});

const dnaPath = path.join(
  __dirname,
  "../dist/distributed-connect-four.dna.json"
);
const dna = Diorama.dna(dnaPath, "distributed-connect-four");

const diorama = new Diorama({
  instances: {
    alice: dna,
    bob: dna
  },
  bridges: [],
  debugLog: false,
  executor: tapeExecutor(require("tape")),
  middleware: backwardCompatibilityMiddleware
});

diorama.registerScenario(
  "Can create and begin a game",
  async (s, t, { alice, bob }) => {
    const create_game_result = await alice.callSync("main", "create_game", {
      opponent: bob.agentId,
      timestamp: 0
    });
    console.log(create_game_result);
    t.equal(create_game_result.Ok.length, 46, "Alice can create a game");

    const move_1_result = await bob.callSync("main", "make_move", {
      new_move: {
        game: create_game_result.Ok,
        move_type: {
          DropPiece: {
            column: 0
          }
        },
        timestamp: 1
      }
    });
    console.log(move_1_result);
    t.equal(
      move_1_result.Err,
      undefined,
      "Bob can make first move in first column"
    );

    const invalid_move = await alice.callSync("main", "make_move", {
      new_move: {
        game: create_game_result.Ok,
        move_type: {
          DropPiece: {
            column: 99
          }
        },
        timestamp: 2
      }
    });
    console.log(invalid_move);
    t.equal(
      invalid_move.Ok,
      undefined,
      "Correctly invalidates out of bounds DropPiece"
    );

    const move_2_result = await alice.callSync("main", "make_move", {
      new_move: {
        game: create_game_result.Ok,
        move_type: {
          DropPiece: {
            column: 0
          }
        },
        timestamp: 3
      }
    });
    console.log(move_2_result);
    t.equal(
      move_2_result.Err,
      undefined,
      "Alice can make her first move after Bob moved"
    );
  }
);

diorama.run();
