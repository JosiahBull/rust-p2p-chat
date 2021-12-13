var searchIndex = JSON.parse('{\
"rust_chat_app":{"doc":"","t":[3,11,11,11,12,11,0,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,5,12,11,12,11,12,5,5,0,12,11,11,11,11,17,3,11,11,12,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,3,13,4,3,13,12,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,12,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11],"n":["Chat","addresses_of_peer","borrow","borrow_mut","dns","from","history","inject_address_change","inject_connected","inject_connection_closed","inject_connection_established","inject_dial_failure","inject_disconnected","inject_event","inject_event","inject_event","inject_expired_external_addr","inject_expired_listen_addr","inject_listen_failure","inject_listener_closed","inject_listener_error","inject_new_external_addr","inject_new_listen_addr","inject_new_listener","into","main","messager","new_handler","peer_id","poll","responder","send_message","send_response","state","state","try_from","try_into","type_id","vzip","HISTORY_SIZE","History","borrow","borrow_mut","data","deserialize","fmt","from","get","get_all","get_count","get_mut","insert","into","new","pointer","serialize","try_from","try_into","type_id","vzip","Message","Message","MessageType","State","State","addressee","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","clone","clone","clone_into","clone_into","data","deserialize","deserialize","deserialize","fmt","fmt","fmt","from","from","from","get_username","history","into","into","into","merge","message_type","serialize","serialize","serialize","source","to_owned","to_owned","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","usernames","vzip","vzip","vzip"],"q":["rust_chat_app","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","rust_chat_app::history","","","","","","","","","","","","","","","","","","","","","rust_chat_app::state","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Send a message using the swarm","Transmit a response to other peers utilising the channels","Handles state for the chat, such as connected users and …","","","","","","","","","","","","","","Get a specific item from history","Get all items from the array in sorted order","Get the current count of items","","Insert an item","","Create a new history","","","","","","","A message or event generated by a user, can contain …","A message sent by a user, containing text that should be …","Represents the type of message being sent, dictating","Contains the current state of the chat application","A message con","The intended recipient of the message, a PeerId encoded as …","","","","","","","","","","","The data contained within the message, represented as a …","","","","","","","","","","Attempt to collect the username of a user, if the user …","The messaging history of the chat","","","","Attempt to merge two states together. Note that if our …","The type of message this is.","","","","The sender of this message, PeerId encoded as a String","","","","","","","","","","","","The usernames of everyone currently connected to the …","","",""],"i":[0,1,1,1,1,1,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,1,1,1,1,1,0,0,0,1,1,1,1,1,0,0,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,0,3,0,0,3,4,3,4,5,3,4,5,3,4,3,4,4,3,4,5,3,4,5,3,4,5,5,5,3,4,5,5,4,3,4,5,4,3,4,3,4,5,3,4,5,3,4,5,5,3,4,5],"f":[null,[[["peerid",3]],[["vec",3,["multiaddr"]],["multiaddr",3]]],[[]],[[]],null,[[]],null,[[["peerid",3],["connectionid",3],["connectedpoint",4]]],[[["peerid",3]]],[[["peerid",3],["connectionid",3],["connectedpoint",4]]],[[["vec",3],["option",4,["vec"]],["peerid",3],["connectionid",3],["connectedpoint",4]]],[[["option",4,["peerid"]],["dialerror",4],["peerid",3]]],[[["peerid",3]]],[[["floodsubevent",4]]],[[["connectionid",3],["peerid",3]]],[[["mdnsevent",4]]],[[["multiaddr",3]]],[[["listenerid",3],["multiaddr",3]]],[[["multiaddr",3]]],[[["listenerid",3],["result",4,["error"]],["error",3]]],[[["listenerid",3],["error",8]]],[[["multiaddr",3]]],[[["listenerid",3],["multiaddr",3]]],[[["listenerid",3]]],[[]],[[],[["result",4,["box"]],["box",3,["error"]]]],null,[[]],null,[[["context",3]],[["poll",4,["networkbehaviouraction"]],["networkbehaviouraction",4]]],null,[[["topic",3],["message",3],["swarm",3]]],[[["message",3],["unboundedsender",3,["message"]]]],null,null,[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]],null,null,[[]],[[]],null,[[],["result",4]],[[["formatter",3]],["result",6]],[[]],[[["usize",15]],["option",4]],[[],["vec",3]],[[],["usize",15]],[[["usize",15]],["option",4]],[[]],[[]],[[]],null,[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]],null,null,null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[],["messagetype",4]],[[],["message",3]],[[]],[[]],null,[[],["result",4]],[[],["result",4]],[[],["result",4]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[]],[[["string",3]],["string",3]],null,[[]],[[]],[[]],[[["state",3]]],null,[[],["result",4]],[[],["result",4]],[[],["result",4]],null,[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],null,[[]],[[]],[[]]],"p":[[3,"Chat"],[3,"History"],[4,"MessageType"],[3,"Message"],[3,"State"]]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};