// automatically generated by the FlatBuffers compiler, do not modify

import * as flatbuffers from 'flatbuffers';



export class User implements flatbuffers.IUnpackableObject<UserT> {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
  __init(i:number, bb:flatbuffers.ByteBuffer):User {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsUser(bb:flatbuffers.ByteBuffer, obj?:User):User {
  return (obj || new User()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsUser(bb:flatbuffers.ByteBuffer, obj?:User):User {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new User()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

/**
 * The user's base58 encoded, 32 byte user ID.
 */
id():string|null
id(optionalEncoding:flatbuffers.Encoding):string|Uint8Array|null
id(optionalEncoding?:any):string|Uint8Array|null {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? this.bb!.__string(this.bb_pos + offset, optionalEncoding) : null;
}

/**
 * The user's name.
 */
name():string|null
name(optionalEncoding:flatbuffers.Encoding):string|Uint8Array|null
name(optionalEncoding?:any):string|Uint8Array|null {
  const offset = this.bb!.__offset(this.bb_pos, 6);
  return offset ? this.bb!.__string(this.bb_pos + offset, optionalEncoding) : null;
}

/**
 * The user's display name. Defaults to the user's name.
 */
displayName():string|null
displayName(optionalEncoding:flatbuffers.Encoding):string|Uint8Array|null
displayName(optionalEncoding?:any):string|Uint8Array|null {
  const offset = this.bb!.__offset(this.bb_pos, 8);
  return offset ? this.bb!.__string(this.bb_pos + offset, optionalEncoding) : null;
}

static startUser(builder:flatbuffers.Builder) {
  builder.startObject(3);
}

static addId(builder:flatbuffers.Builder, idOffset:flatbuffers.Offset) {
  builder.addFieldOffset(0, idOffset, 0);
}

static addName(builder:flatbuffers.Builder, nameOffset:flatbuffers.Offset) {
  builder.addFieldOffset(1, nameOffset, 0);
}

static addDisplayName(builder:flatbuffers.Builder, displayNameOffset:flatbuffers.Offset) {
  builder.addFieldOffset(2, displayNameOffset, 0);
}

static endUser(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}

static createUser(builder:flatbuffers.Builder, idOffset:flatbuffers.Offset, nameOffset:flatbuffers.Offset, displayNameOffset:flatbuffers.Offset):flatbuffers.Offset {
  User.startUser(builder);
  User.addId(builder, idOffset);
  User.addName(builder, nameOffset);
  User.addDisplayName(builder, displayNameOffset);
  return User.endUser(builder);
}

unpack(): UserT {
  return new UserT(
    this.id(),
    this.name(),
    this.displayName()
  );
}


unpackTo(_o: UserT): void {
  _o.id = this.id();
  _o.name = this.name();
  _o.displayName = this.displayName();
}
}

export class UserT implements flatbuffers.IGeneratedObject {
constructor(
  public id: string|Uint8Array|null = null,
  public name: string|Uint8Array|null = null,
  public displayName: string|Uint8Array|null = null
){}


pack(builder:flatbuffers.Builder): flatbuffers.Offset {
  const id = (this.id !== null ? builder.createString(this.id!) : 0);
  const name = (this.name !== null ? builder.createString(this.name!) : 0);
  const displayName = (this.displayName !== null ? builder.createString(this.displayName!) : 0);

  return User.createUser(builder,
    id,
    name,
    displayName
  );
}
}