const stringToUint8Array = (str) => {
    if(typeof(str) != "string"){
      return undefined;
    }

    let seq = [];
    for(let i = 0; i < str.length; i++){
      seq.push(str.charCodeAt(i));
    }
    return new Uint8Array(seq);
}

// convert a Uint8Array to a string
const uint8ArrayToString = (bytes) => {
    let str = "";
    bytes.forEach( (byte) => {
        str += String.fromCharCode(byte);
    });
    return str;
}

// ads 'amount' to each byte in the array
const upshift = (bytes, amount) => {
    for(let i = 0; i < bytes.length; i++){
        bytes[i] += amount;
    }
    return bytes;
}

// subtracts 'amount' from each byte in the array
const downshift = (bytes, amount) => {
    for(let i = 0; i < bytes.length; i++){
        bytes[i] -= amount;
    }
    return bytes;
}

const encrypt = (digest, key) => {
    return uint8ArrayToString(upshift(stringToUint8Array(digest), key));
}

const decrypt = (digest, key) => {
    return uint8ArrayToString(downshift(stringToUint8Array(digest), key));
}

export const Cypher = {
    encrypt: encrypt,
}