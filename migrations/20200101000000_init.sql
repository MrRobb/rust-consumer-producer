CREATE TABLE "Recipes" (
    "id" bigserial primary key,
    "name" text not null,
    "description" text,
    "ingredients" bytea
);
