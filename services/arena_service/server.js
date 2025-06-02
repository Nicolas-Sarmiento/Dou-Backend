import dotenv from 'dotenv';
import express from "express";
import http from "http";
import { WebSocketServer } from "ws";
import { handleWebSocketConnection } from "./web_socket_handler/websocketHandler.mjs";

dotenv.config();

const PORT = 8000;

const app = express();
const server = http.createServer(app);
const wss = new WebSocketServer({ server });

wss.on("connection", (ws) => {
  handleWebSocketConnection(ws);
});

server.listen(PORT, () => {
  console.log(`Arena server running on: ${PORT}`);
});
