import { PrismaClient } from "@prisma/client";

if (process.env.NODE_ENV !== "production") {
  if (!global.prismaGlobal) {
    global.prismaGlobal = new PrismaClient({
      datasourceUrl: "file:dev.sqlite"
    });
  }
}

const prisma = global.prismaGlobal ?? new PrismaClient({
  datasourceUrl: "file:dev.sqlite"
});

export default prisma;
