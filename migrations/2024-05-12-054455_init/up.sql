-- Your SQL goes here
CREATE TABLE "voting"(
	"id" UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
	"title" VARCHAR NOT NULL,
	"description" TEXT NOT NULL,
	"options" TEXT[] NOT NULL,
	"active" BOOL NOT NULL DEFAULT FALSE
);

CREATE TABLE "vote"(
	"id" UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
	"challenge" BYTEA NOT NULL,
	"responses" BYTEA NOT NULL,
	"encrypted" BYTEA NOT NULL,
    "key_image" BYTEA NOT NULL,
    "nonce" BYTEA NOT NULL,
    "key" BYTEA,
    "voting_id" UUID NOT NULL REFERENCES "voting"("id") ON DELETE CASCADE
);