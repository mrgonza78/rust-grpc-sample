import * as jspb from 'google-protobuf'



export class Message extends jspb.Message {
  getContent(): string;
  setContent(value: string): Message;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Message.AsObject;
  static toObject(includeInstance: boolean, msg: Message): Message.AsObject;
  static serializeBinaryToWriter(message: Message, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Message;
  static deserializeBinaryFromReader(message: Message, reader: jspb.BinaryReader): Message;
}

export namespace Message {
  export type AsObject = {
    content: string,
  }
}

export class SendMessageResponse extends jspb.Message {
  getMessagesProcessed(): number;
  setMessagesProcessed(value: number): SendMessageResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SendMessageResponse.AsObject;
  static toObject(includeInstance: boolean, msg: SendMessageResponse): SendMessageResponse.AsObject;
  static serializeBinaryToWriter(message: SendMessageResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SendMessageResponse;
  static deserializeBinaryFromReader(message: SendMessageResponse, reader: jspb.BinaryReader): SendMessageResponse;
}

export namespace SendMessageResponse {
  export type AsObject = {
    messagesProcessed: number,
  }
}

export class HistoryRequest extends jspb.Message {
  getStartingAt(): number;
  setStartingAt(value: number): HistoryRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): HistoryRequest.AsObject;
  static toObject(includeInstance: boolean, msg: HistoryRequest): HistoryRequest.AsObject;
  static serializeBinaryToWriter(message: HistoryRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): HistoryRequest;
  static deserializeBinaryFromReader(message: HistoryRequest, reader: jspb.BinaryReader): HistoryRequest;
}

export namespace HistoryRequest {
  export type AsObject = {
    startingAt: number,
  }
}

