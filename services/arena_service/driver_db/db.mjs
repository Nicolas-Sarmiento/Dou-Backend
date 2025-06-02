import { Sequelize } from "sequelize";

const sequelize = new Sequelize(process.env.DATABASE_URL, {
  dialect: 'postgres',
  retry: {
    max: 30
  },
  logging: false
});

(async () => {
  try {
    await sequelize.authenticate();
  } catch (error) {
    console.error("Error al conectar con PostgreSQL:", error);
    process.exit(1);
  }
})();

export default sequelize;