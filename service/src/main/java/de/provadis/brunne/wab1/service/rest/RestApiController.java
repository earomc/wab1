package de.provadis.brunne.wab1.service.rest;

import de.provadis.brunne.wab1.service.common.DataSource;
import de.provadis.brunne.wab1.service.common.datamodel.Customer;
import de.provadis.brunne.wab1.service.common.datamodel.Order;
import de.provadis.brunne.wab1.service.common.datamodel.PriceFilter;
import de.provadis.brunne.wab1.service.common.datamodel.Product;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;

import java.util.List;

@RestController
public class RestApiController {
    @Autowired
    private DataSource dataSource;

    // CUSTOMERS
    @GetMapping("/customer")
    public Customer getCustomerById(@RequestParam String id) {
        return dataSource.getCustomerById(id);
    }

    @GetMapping("/customers")
    public List<Customer> getCustomers() {
        return dataSource.getCustomers();
    }

    // PRODUCTS
    @GetMapping("/product")
    public Product getProductById(@RequestParam String id) {
        return dataSource.getProductById(id);
    }
    @GetMapping("/products")
    public List<Product> getProducts(@RequestParam(required = false) Float min,
                                     @RequestParam(required = false) Float max) {
        if (min == null && max == null) {
            return dataSource.getProducts();
        }
        return dataSource.getProducts(new PriceFilter(min, max));
    }

    // ORDERS
    @GetMapping("/orders")
    public List<Order> getOrders(@RequestParam(required = false) String customerId) {
        if (customerId != null) {
            return dataSource.getCustomerById(customerId).getOrders();
        } else {
            return dataSource.getOrders();
        }
    }

    @GetMapping("/order")
    public Order getOrderById(@RequestParam String id) {
        return dataSource.getOrderById(id);
    }
}
