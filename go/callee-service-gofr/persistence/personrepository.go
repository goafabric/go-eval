package persistence

import (
	"gofr.dev/pkg/gofr"
)

func RoutePerson(app *gofr.App) {
    app.GET("/persons", func(ctx *gofr.Context) (interface{}, error) {
        err := DemoData(ctx)
        if err != nil {
            return nil, err
        }
        return findAll(ctx)
	})
}

func findAll(ctx *gofr.Context) (any, error) {
    var persons []Person
    
	rows, err := ctx.SQL.QueryContext(ctx, "SELECT id, first_name, last_name FROM example_0.person")
    if err != nil {
        return nil, err
    }

    for rows.Next() {
        var person Person
        if err := rows.Scan(&person.ID, &person.FirstName, &person.LastName); err != nil {
            return nil, err
        }

        persons = append(persons, person)
    }
    return persons, nil

}