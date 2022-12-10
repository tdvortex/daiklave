#[test]
fn lifecycle() {
    // User inputs a username, Client serializes it
    // Server deserializes it and creates a new player
    // Server serializes player result and sends it back to the client
    // Client deserializes and extracts player ID
    // Client (in isolation) creates a character and subcomponents
    // Client builds, serializes, and sends to server
    // Server deserializes, inserts, then extracts the character
    // Server serializes and sends character to client
    // Client deserializes character and modifies it
    // Client reserializes character and sends to server
    // Server deserializes, reconciles, inserts, extracts, and reserializes
    // Client deserializes
    // Client sends delete player order
    // Server deletes player, sends confirmation
    // End state: non-custom elements remain in database
    // Clean up database to end test
}