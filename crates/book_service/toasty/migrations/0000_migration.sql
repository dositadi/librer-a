CREATE TABLE "books" (
    "id" UUID NOT NULL,
    "published_date" DATE NOT NULL,
    "status" SMALLINT NOT NULL,
    "title" TEXT NOT NULL,
    "description" TEXT,
    "image_url" TEXT,
    "created_at" TIMESTAMPTZ(6) NOT NULL,
    "updated_at" TIMESTAMPTZ(6) NOT NULL,
    PRIMARY KEY ("id")
);
