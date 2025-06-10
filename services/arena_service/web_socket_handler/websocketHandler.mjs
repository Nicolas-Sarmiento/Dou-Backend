import { queue, rooms, createRoom } from "../matchmaking/matchmaking.mjs";

const activeUsers = new Set();

function startKeepAlive(ws) {
  const interval = setInterval(() => {
    if (ws.readyState === ws.OPEN) {
      ws.send(JSON.stringify({ type: "ping" }));
    }
  }, 30000);

  ws.keepAliveInterval = interval;
}

function stopKeepAlive(ws) {
  if (ws.keepAliveInterval) {
    clearInterval(ws.keepAliveInterval);
  }
}

export function handleWebSocketConnection(ws) {
  ws.ready = false;

  ws.on("message", (data) => {
    let message;

    try {
      message = JSON.parse(data);
    } catch (error) {
      ws.send(JSON.stringify({ type: "error", message: "Formato JSON inválido." }));
      return;
    }

    if (message.type === "join") {
      ws.userId = message.userId;

      if (activeUsers.has(ws.userId)) {
        ws.send(JSON.stringify({ type: "error", message: "Usuario ya conectado." }));
        return;
      }

      queue.push(ws);
      activeUsers.add(ws.userId);

      if (queue.length >= 2) {
        const userA = queue.shift();
        const userB = queue.shift();
        createRoom(userA, userB);
      }

    } else if (message.type === "submission") {
      if (!ws.roomId || !rooms.has(ws.roomId)) {
        ws.send(JSON.stringify({ type: "error", message: "Sala no encontrada o aún no asignada." }));
        return;
      }

      if (message.roomId !== ws.roomId) {
        ws.send(JSON.stringify({ type: "error", message: "No estás autorizado para enviar datos a esta sala." }));
        return;
      }

      const room = rooms.get(ws.roomId);
      const opponent = room.users.find(u => u !== ws);

      opponent.send(JSON.stringify({
        type: "submission",
        from: ws.userId
      }));

    } else if (message.type === "verdict") {

      const room = rooms.get(ws.roomId);
      const value = rooms.get(ws.value); 
      const opponent = room.users.find(u => u !== ws);

      opponent.send(JSON.stringify({
        type: "verdict",
        from: ws.userId,
        verdict: message.verdict,
        verdict_value: value
      }));

      if (message.verdict === "AC") {
        room.users.forEach(u => {
          u.send(JSON.stringify({
            type: "winner",
            winner: ws.userId
          }));
          u.close();
        });

        rooms.delete(ws.roomId);
      }
    }
  });

  ws.on("close", () => {
    stopKeepAlive(ws); 

    if (ws.userId && activeUsers.has(ws.userId)) {
      activeUsers.delete(ws.userId);
    }

    const indexInQueue = queue.indexOf(ws);
    if (indexInQueue !== -1) {
      queue.splice(indexInQueue, 1);
    }

    if (ws.roomId && rooms.has(ws.roomId)) {
      const room = rooms.get(ws.roomId);
      room.users.forEach(u => {
        if (u !== ws) u.send(JSON.stringify({ type: "opponent_disconnected" }));
        u.close();
      });
      rooms.delete(ws.roomId);
    }
  });

  startKeepAlive(ws);
}

