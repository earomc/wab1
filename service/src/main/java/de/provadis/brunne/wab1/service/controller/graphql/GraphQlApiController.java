package de.provadis.brunne.wab1.service.controller.graphql;

import de.provadis.brunne.wab1.service.DataSource;
import de.provadis.brunne.wab1.service.datamodel.Customer;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.graphql.data.method.annotation.Argument;
import org.springframework.graphql.data.method.annotation.QueryMapping;
import org.springframework.stereotype.Controller;

@Controller
public class GraphQlApiController {

    @Autowired
    private DataSource dataSource;

    @QueryMapping
    public Customer customer(@Argument String id) {
        return dataSource.findCustomerById(id);
    }
}
