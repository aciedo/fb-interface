// automatically generated by the FlatBuffers compiler, do not modify

import * as flatbuffers from 'flatbuffers';



export class RegisterNumberRes implements flatbuffers.IUnpackableObject<RegisterNumberResT> {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
  __init(i:number, bb:flatbuffers.ByteBuffer):RegisterNumberRes {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsRegisterNumberRes(bb:flatbuffers.ByteBuffer, obj?:RegisterNumberRes):RegisterNumberRes {
  return (obj || new RegisterNumberRes()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsRegisterNumberRes(bb:flatbuffers.ByteBuffer, obj?:RegisterNumberRes):RegisterNumberRes {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new RegisterNumberRes()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

multiplier():number {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? this.bb!.readUint16(this.bb_pos + offset) : 0;
}

static startRegisterNumberRes(builder:flatbuffers.Builder) {
  builder.startObject(1);
}

static addMultiplier(builder:flatbuffers.Builder, multiplier:number) {
  builder.addFieldInt16(0, multiplier, 0);
}

static endRegisterNumberRes(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}

static createRegisterNumberRes(builder:flatbuffers.Builder, multiplier:number):flatbuffers.Offset {
  RegisterNumberRes.startRegisterNumberRes(builder);
  RegisterNumberRes.addMultiplier(builder, multiplier);
  return RegisterNumberRes.endRegisterNumberRes(builder);
}

unpack(): RegisterNumberResT {
  return new RegisterNumberResT(
    this.multiplier()
  );
}


unpackTo(_o: RegisterNumberResT): void {
  _o.multiplier = this.multiplier();
}
}

export class RegisterNumberResT implements flatbuffers.IGeneratedObject {
constructor(
  public multiplier: number = 0
){}


pack(builder:flatbuffers.Builder): flatbuffers.Offset {
  return RegisterNumberRes.createRegisterNumberRes(builder,
    this.multiplier
  );
}
}