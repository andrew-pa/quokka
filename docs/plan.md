# Quokka overview

Server facilitates transfer of messages between entities

A message contains:

+ the sender's id
+ the recipient's id
+ the type of content it holds (MIME?)
+ the actual content binary

## Protocol between Client and Server

+ User Creation?

	Arguments:	User ID; raw auth info, profile info?

	Response:	auth info [ encryption keys? ]

+ Init
	
	Arguments:	User ID; authentication information

	Response:	more authentication info?

+ Send Message
	
	Arguments:	A message as described above

	Response:	ack?

+ Status

	Arguments:	Client status [ currently looking/typing ⇔ not looking ]

	Response:	New messages, other users status?

+ User info query?

	Arguments:	other User ID

	Response:	？？？Profile info？？？

## Federation

User ids with different specified hostname

Server proxies message to other server? Or does client handle sending to different hosts?

## IRC

Use an IRC to facilitate exchange of messages between clients

Better than RESTful API; REST really shouldn't be used for serverside programming, as the client is constantly pulling from the server 

?? Open source IRC readily available on github? 

This is intresting, but if one is trying to write IRC and the solution is to use existing IRC, there is something mildly tautological about the whole business 

## To Be Resolved
- Federation
- fully spec the protocol
- Group messages? should be easy; it's just a special entity
- Scaling to large amounts of users. >1000? >1,000,000?; Large groups?
