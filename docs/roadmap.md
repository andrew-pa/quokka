
# Roadmap #

Indices show temporal relationship wrt ordering of completion

+ [0] Define a basic Client <=> Server protocol
	
	- Doesn't need to be complete, just enough to send and receive messages
	- *Can we ignore security or should we focus on it from the onset?*

+ [1] Implement the protocol for the server
+ [1] Implement the protocol on the 'CLI' client

	- the UX for this client is, well, irrelevant, but also undecided
	- it really exists as a way to test the server, so that should be the main focus
	- (it's possible that a test framework could be rigged up.....)

+ [2] Improve protocol (security, optomization, features) and implement them
	
	+ [2] Part of improving the protocol could be making a proper test framework for the server that integrates nicely with Rust's existing testing stuff (`cargo test` etc)

+ [2] Proper clients (desktop/Android)
+ [3] Support hosting on cloud (Heroku/AWS)
+ [3] Push notifications
+ [3] iOS client
