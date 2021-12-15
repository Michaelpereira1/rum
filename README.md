# rum
Universal Virtual Machine
 
Michael Pereira, Hunter Larkin

We acknowledge that we received help from the TA’s

The machine itself was properly implemented, with every instruction working as detailed in the assignment handout. Alongside this, the general architecture was implemented correctly as well.

The major departures from our original design include the use of a hashmap instead of a 2d array to represent the memory segment and their identifiers, the general purpose registers being stored inside of a fixed size array instead of a vector, and general file structure.

Our modules include a machine.rs file, a lib.rs file, and a load.rs file. The load.rs file contains a single function that reads through the input file and returns the instruction word. It only knows what resides in that function itself and is kept in the dark regarding the I/O method and the UM instructions. The main.rs file contains the necessary code to read an argument from the command line (um files) and calls our load function talked about previously. It is only aware of the code necessary to get our command line argument and start the machine. It is unaware of the UM instructions and their helper functions. The machine.rs file contains the meat of the Universal Machine and holds all the code necessary for the various instructions used. It is unaware of anything regarding loading the initial 32-bit instruction word from a file and reading anything from the command line. 

It takes our UM 1.17 seconds to execute 50 million instructions. We know this because midmark.um has 85070522 instructions, which took 2 seconds to execute.
2/85070522 * 50,000,000 = 1.17 seconds

Hours spent analyzing the problem: ~ 4
Hours spent thinking of the design: ~ 4
Hours spent implementing the assignment: ~10-15