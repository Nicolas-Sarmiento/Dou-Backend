import { getRandomProblemId } from '../driver_db/db_interact.mjs'

export const queue = [];
export const rooms = new Map();

function generateRoomId() {
  return Math.random().toString(36).substring(2, 9);
}

function  assignProblem() {
    return  getRandomProblemId();
}

export function createRoom(userA, userB) {
  const roomId = generateRoomId();

  assignProblem().then(problemId => {
    const room = { users: [userA, userB], problemId };
    rooms.set(roomId, room);

    userA.roomId = roomId;
    userB.roomId = roomId;

    userA.ready = true;
    userB.ready = true;

    userA.send(JSON.stringify({
      type: "start",
      roomId,
      problemId,
      opponent: userB.userId
    }));

    userB.send(JSON.stringify({
      type: "start",
      roomId,
      problemId,
      opponent: userA.userId
    }));
  });
}

