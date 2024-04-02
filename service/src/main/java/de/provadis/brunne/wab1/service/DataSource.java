package de.provadis.brunne.wab1.service;

import de.provadis.brunne.wab1.service.datamodel.Customer;
import de.provadis.brunne.wab1.service.datamodel.Order;
import de.provadis.brunne.wab1.service.datamodel.PriceFilter;
import de.provadis.brunne.wab1.service.datamodel.Product;
import org.springframework.stereotype.Component;

import java.util.HashMap;
import java.util.List;
import java.util.Map;

@Component
public class DataSource {
    private final Map<String, Customer> customers = new HashMap<>();
    private final Map<String, Order> orders = new HashMap<>();

    private final Map<String, Product> products = new HashMap<>();

    public DataSource() {
        initProducts();
        initCustomers();
    }

    public void initProducts() {
        addProducts(new Product("product-1",
                        "500g Spirelli",
                        "Aus Hartweizengrieß",
                        1.5),
                new Product("product-2",
                        "Kilo Teig",
                        "Teig halt",
                        1.75),
                new Product("product-3",
                        "Wecker",
                        "weckt",
                        24.99
                ),
                new Product("product-4",
                        "Pizzaofen",
                        "backt Pizza",
                        599.0));
    }

    private void addCustomers(Customer... customers) {
        for (Customer customer : customers) {
            this.customers.put(customer.getId(), customer);
        }
    }

    private void addProducts(Product... products) {
        for (Product product : products) {
            this.products.put(product.id(), product);
        }
    }

    private void initCustomers() {
        this.addCustomers(
                new Customer("customer-1", "Richie Langston", "richielangston@rockstarmail.com", "1231 Sesame St, Aberdeen, WA, United States"),
                new Customer("customer-2", "Maine Boole", "greatmail@coolmail.me", "the only amazing home address of maine boole"),
                new Customer("customer-3", "Mr. Tim Poorstein", "nomailfortim@myprov.de", "Schrankstraße 2, 12314 Möbelheim, Deutschland"),
                new Customer("customer-4", "Sanoneinan Anaaahrijey", "sanoneinan.anaaahrijey@gmail.com", "Wurstfabrikstraße 2"),
                new Customer("customer-5", "Simon S. Colorful", "simoncolorful@paramail.ch", "mom"),
                new Customer("customer-6", "Morris Well", "nope@no.com", "Falafelgasse 1501")
        );
        addOrder(customers.get("customer-3"),
                new Order("order-1",
                        List.of(products.get("product-2"))
                ));
        addOrder(customers.get("customer-6"),
                new Order("order-2", List.of(
                        products.get("product-3"),
                        products.get("product-4")
                )));
    }

    public List<Order> getOrders() {
        return orders.values().stream().toList();
    }

    public List<Customer> getCustomers() {
        return customers.values().stream().toList();
    }

    public List<Product> getProducts() {
        return products.values().stream().toList();
    }

    public List<Product> getProducts(PriceFilter filter) {
        return products.values().stream().filter(product -> {
            if (filter.max() != null && product.price() > filter.max()) {
                return false;
            }
            //noinspection RedundantIfStatement
            if (filter.min() != null && product.price() < filter.min()) {
                return false;
            }
            return true;
        }).toList();
    }

    public void addOrder(Customer customer, Order order) {
        customer.getOrders().add(order);
        orders.put(order.getId(), order);
    }

    public Order getOrderById(String id) {
        return orders.get(id);
    }

    public Customer getCustomerById(String id) {
        return customers.get(id);
    }

    public Product getProductById(String id) {
        return products.get(id);
    }

}
