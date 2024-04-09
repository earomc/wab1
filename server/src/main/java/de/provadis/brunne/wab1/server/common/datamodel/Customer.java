package de.provadis.brunne.wab1.server.common.datamodel;

import java.util.ArrayList;
import java.util.List;

public record Customer(String id, String name, String email, String address, List<Order> orders){
    public Customer(String id, String name, String email, String address) {
        this(id, name, email, address, new ArrayList<>());
    }
}