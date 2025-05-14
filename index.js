class User {
  constructor(username, password) {
    this.username = username;
    this.password = password;
  }

  serialize() {
    // Simple serialization: convert to JSON and then to Uint8Array
    const jsonStr = JSON.stringify(this); //JSON.stringify() is used to convert the object to a JSON string 
    const encoder = new TextEncoder(); //TextEncoder() is used to convert the JSON string to a Uint8Array
    return encoder.encode(jsonStr); //encode() is used to convert the JSON string to a Uint8Array
  }

  static deserialize(bytes) {
    // Simple deserialization: convert from Uint8Array to JSON to object
    const decoder = new TextDecoder(); //TextDecoder() is used to convert the Uint8Array to a JSON string
    const jsonStr = decoder.decode(bytes); //decode() is used to convert the Uint8Array to a JSON string
    const data = JSON.parse(jsonStr); //JSON.parse() is used to convert the JSON string to an object
    return new User(data.username, data.password); //return the object
  }
}

function main() {
  const user = new User("Shorya", "123456");

  // Serialize
  const serializedData = user.serialize();
  console.log("Serialized:", serializedData);

  // Deserialize
  try {
    const deserializedUser = User.deserialize(serializedData);
    console.log("Deserialized:", deserializedUser);
  } catch (e) {
    console.log("Error:", e);
  }
}

main();
