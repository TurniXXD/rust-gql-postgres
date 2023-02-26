-- Your SQL goes here
CREATE TABLE users (
  id   SERIAL  PRIMARY KEY,
  translation VARCHAR NOT NULL
);

CREATE TABLE posts (
  id    SERIAL PRIMARY KEY,
  user_id SERIAL REFERENCES users(id),
  title VARCHAR NOT NULL,
  body  VARCHAR,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE TABLE "components" (
  "id" SERIAL NOT NULL,
  "name" TEXT NOT NULL,
  "translations_id" integer NOT NULL,
  CONSTRAINT "components_pk" PRIMARY KEY ("id")
) WITH (OIDS = FALSE);

CREATE TABLE "translations" (
  "id" SERIAL NOT NULL,
  CONSTRAINT "translations_pk" PRIMARY KEY ("id")
) WITH (OIDS = FALSE);

CREATE TABLE "translations_packs" (
  "id" SERIAL NOT NULL,
  "lang" TEXT NOT NULL,
  "translation" TEXT NOT NULL,
  "translation_id" integer NOT NULL,
  CONSTRAINT "translation_packs_pk" PRIMARY KEY ("id")
) WITH (OIDS = FALSE);

ALTER TABLE "translations_packs"
ADD CONSTRAINT "translations_packs_lang_translation_id_key" UNIQUE ("lang", "translation_id");

ALTER TABLE "translations_packs" ADD CONSTRAINT "translations_packs_fk0" FOREIGN KEY ("translation_id") REFERENCES "translations"("id");

ALTER TABLE "components" ADD CONSTRAINT "components_fk0" FOREIGN KEY ("translations_id") REFERENCES "translations"("id");

CREATE VIEW post_details AS
SELECT 
p.id,
u.translation,
p.title,
p.body,
p.created_at
FROM posts p
INNER JOIN users u
on p.user_id = u.id
;