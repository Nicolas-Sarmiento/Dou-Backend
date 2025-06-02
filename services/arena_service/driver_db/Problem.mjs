import { Model, DataTypes } from "sequelize";
import sequelize from "./db.mjs";  

class Problem extends Model {}

Problem.init({
  problemId: {
    field: 'PROBLEM_ID',
    type: DataTypes.INTEGER,
    primaryKey: true,
    autoIncrement: true,
  },
  problemName: {
    field: 'PROBLEM_NAME',
    type: DataTypes.STRING(100),
    unique: true,
    allowNull: false,
  },
  problemStatementUrl: {
    field: 'PROBLEM_STATEMENT_URL',
    type: DataTypes.TEXT,
    allowNull: false,
  },
  problemTestCasesUrl: {
    field: 'PROBLEM_TEST_CASES_URL',
    type: DataTypes.TEXT,
    allowNull: false,
  },
  problemOutputsUrl: {
    field: 'PROBLEM_OUTPUTS_URL',
    type: DataTypes.TEXT,
    allowNull: false,
  },
  problemMemoryMbLimit: {
    field: 'PROBLEM_MEMORY_MB_LIMIT',
    type: DataTypes.INTEGER,
    allowNull: false,
  },
  problemTimeMsLimit: {
    field: 'PROBLEM_TIME_MS_LIMIT',
    type: DataTypes.INTEGER,
    allowNull: false,
  },
}, {
  sequelize,
  modelName: "Problem",
  tableName: "problems",
  timestamps: false, 
  underscored: false, 
});

await sequelize.sync();
export default Problem;
