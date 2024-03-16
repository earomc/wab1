package de.provadis.brunne.wab1.service.controller.graphql;

import de.provadis.brunne.wab1.service.DataSource;
import de.provadis.brunne.wab1.service.datamodel.Customer;
import de.provadis.brunne.wab1.service.datamodel.Order;
import de.provadis.brunne.wab1.service.datamodel.Product;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.graphql.data.method.annotation.Argument;
import org.springframework.graphql.data.method.annotation.QueryMapping;
import org.springframework.graphql.data.method.annotation.SchemaMapping;
import org.springframework.stereotype.Controller;

import java.util.List;

@Controller
public class GraphQlApiController {

    @Autowired
    private DataSource dataSource;

    @QueryMapping
    public Customer customer(@Argument String id) {
        return dataSource.getCustomerById(id);
    }

    @QueryMapping
    public List<Customer> customers() {
        return dataSource.getCustomers();
    }

    @QueryMapping
    public List<Product> products() {
        return dataSource.getProducts();
    }

    @QueryMapping
    public Product product(@Argument String id) {
        return dataSource.getProductById(id);
    }


    @QueryMapping
    public List<Order> orders() {
        return dataSource.getOrders();
    }

    @QueryMapping
    public Order order(@Argument String id) {
        return dataSource.getOrderById(id);
    }
}
