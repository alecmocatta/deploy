//= {
//=   "output": {
//=     "2": [
//=       "",
//=       true
//=     ],
//=     "1": [
//=       "hi\nhi\n",
//=       true
//=     ]
//=   },
//=   "children": [
//=     {
//=       "output": {
//=         "2": [
//=           "",
//=           true
//=         ],
//=         "1": [
//=           "",
//=           true
//=         ]
//=       },
//=       "children": [],
//=       "exit": "Success"
//=     },
//=     {
//=       "output": {
//=         "2": [
//=           "",
//=           true
//=         ],
//=         "1": [
//=           "",
//=           true
//=         ]
//=       },
//=       "children": [],
//=       "exit": "Success"
//=     }
//=   ],
//=   "exit": "Success"
//= }

use constellation::*;

fn main() {
	init(Resources {
		mem: 20 * Mem::MIB,
		..Resources::default()
	});
	for _ in 0..2 {
		let pid = spawn(
			Resources {
				mem: 20 * Mem::MIB,
				..Resources::default()
			},
			FnOnce!(|parent| {
				let sender = Sender::<String>::new(parent);
				sender.send(String::from("hi")).block();
			}),
		)
		.block()
		.expect("spawn() failed to allocate process");
		let receiver = Receiver::<String>::new(pid);
		println!("{}", receiver.recv().block().unwrap());
	}
}
