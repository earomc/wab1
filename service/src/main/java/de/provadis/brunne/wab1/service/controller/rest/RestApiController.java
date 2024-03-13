package de.provadis.brunne.wab1.service.controller.rest;

import de.provadis.brunne.wab1.service.DataSource;
import de.provadis.brunne.wab1.service.datamodel.Customer;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;

import java.util.List;

@RestController
public class RestApiController {
    @Autowired
    private DataSource dataSource;
    @GetMapping("/customer")
    public Customer getById(@RequestParam String id) {
        return dataSource.findCustomerById(id);
    }

    @GetMapping("/customers")
    public List<Customer> getAllCustomers() {
        return dataSource.getAllCustomers();
    }
}
