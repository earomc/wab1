package de.provadis.brunne.wab1.service.graphql;

import de.provadis.brunne.wab1.service.common.DataSource;
import de.provadis.brunne.wab1.service.common.datamodel.Customer;
import de.provadis.brunne.wab1.service.common.datamodel.Order;
import de.provadis.brunne.wab1.service.common.datamodel.PriceFilter;
import de.provadis.brunne.wab1.service.common.datamodel.Product;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.graphql.data.method.annotation.Argument;
import org.springframework.graphql.data.method.annotation.QueryMapping;
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
    public Product product(@Argument String id) {
        return dataSource.getProductById(id);
    }

    @QueryMapping
    public List<Product> products(@Argument PriceFilter priceFilter) {
        if (priceFilter != null) {
            return dataSource.getProducts(priceFilter);
        }
        return dataSource.getProducts();
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
