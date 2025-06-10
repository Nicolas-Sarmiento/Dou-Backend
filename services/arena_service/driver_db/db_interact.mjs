import Problem from "./Problem.mjs";
import sequelize from "./db.mjs";

export async function getRandomProblemId() {

    const randomProblem = await Problem.findOne({
        attributes: ['problem_id'],
        order: sequelize.random(),
        raw: true,
    });
    return randomProblem ? randomProblem.problem_id : null;
}