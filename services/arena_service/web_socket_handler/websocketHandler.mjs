import { queue, rooms, createRoom } from "../matchmaking/matchmaking.mjs";

export function handleWebSocketConnection(ws) {
  ws.on("message", (data) => {
    const message = JSON.parse(data);

    //mensaje vacío o error de parseo.

    if (message.type === "join") {
      ws.userId = message.userId;
      //Validar no inserta doble ID
      queue.push(ws);


      if (queue.length >= 2) {
        const userA = queue.shift();
        const userB = queue.shift();
        createRoom(userA, userB);
      }
    } else if (message.type === "submission") {
      const room = rooms.get(ws.roomId);
      if (!room) return;
      //validar room

      const opponent = room.users.find(u => u !== ws);
      opponent.send(JSON.stringify({
        type: "submission",
        from: ws.userId
      }));
    } else if (message.type === "verdict") {
      const room = rooms.get(ws.roomId);
      const value = rooms.get(ws.value);
      if (!room) return;
      //validar room existente

      const opponent = room.users.find(u => u !== ws);

      opponent.send(JSON.stringify({
        type: "verdict",
        from: ws.userId,
        verdict: message.verdict,
        verdict_value: value
      }));

      if (message.verdict === "AC") {
        room.users.forEach(u => u.send(JSON.stringify({
          type: "winner",
          winner: ws.userId
        })));
        rooms.delete(ws.roomId);
      }
    }
  });

  ws.on("close", () => {
    if (ws.roomId && rooms.has(ws.roomId)) {
      const room = rooms.get(ws.roomId);
      room.users.forEach(u => {
        if (u !== ws) u.send(JSON.stringify({ type: "opponent_disconnected" }));
      });
      rooms.delete(ws.roomId);
    }
  });
}
