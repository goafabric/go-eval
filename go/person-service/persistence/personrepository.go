package persistence

import (
	"person-service/controller/dto"

	"gofr.dev/pkg/gofr"
)

func FindAll(ctx *gofr.Context, firstName string, lastName string) (any, error) {
	var persons []dto.Person

	var sql = "SELECT id, first_name, last_name FROM example_0.person"

	if firstName != "" {
		sql += " WHERE first_name = '" + firstName + "'"
	}

	if lastName != "" {
		sql += " WHERE last_name = '" + lastName + "'"
	}

	rows, err := ctx.SQL.QueryContext(ctx, sql)
	if err != nil {
		return nil, err
	}

	for rows.Next() {
		var person dto.Person
		if err := rows.Scan(&person.ID, &person.FirstName, &person.LastName); err != nil {
			return nil, err
		}

		persons = append(persons, person)
	}
	return persons, nil

}
