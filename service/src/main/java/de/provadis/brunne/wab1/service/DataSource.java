package de.provadis.brunne.wab1.service;

import de.provadis.brunne.wab1.service.datamodel.Customer;
import de.provadis.brunne.wab1.service.datamodel.Order;
import org.springframework.stereotype.Component;

import java.util.ArrayList;
import java.util.List;
import java.util.UUID;

@Component
public class DataSource {
    private final List<Customer> customers;

    public DataSource() {
        this.customers = new ArrayList<>();
        this.customers.addAll(List.of(
                new Customer("89440fef-494d-43b3-b600-37f7bda877c1",
                        "Richie Langston",
                        "richielangston@mail.com",
                        "1231 Sesame St, Aberdeen, WA, United States"),
                new Customer("04d1c23e-8268-4690-9b71-44cad5f959c8",
                        "Maine Boole",
                        "greatmail@examplemail.me",
                        "the amazing home address of maine boole"),
                new Customer("68e49e58-b5be-4303-bb26-6ec75f168b65",
                        "Mr. Tim Poorstein",
                        "nononomailfortim@myprov.de",
                        "Schrankstraße 2, 12314 Möbelheim, Deutschland"
                ),
                new Customer("8d8db92b-49b8-4b31-b1b4-3aea6915d7a1",
                        "Sanoneinan Anaaahrijey",
                        "sanoneinan.anaaahrijey@gmail.com",
                        "Wurstfabrikstraße 2")
        ));
    }

    public List<Order> getOrders() {

    }

    public Customer findCustomerById(String id) {
        for (Customer customer : customers) {
            if (customer.id().equals(id)) {
                return customer;
            }
        }
        return null;
    }

    public List<Customer> getAllCustomers() {
        return customers;
    }
}
