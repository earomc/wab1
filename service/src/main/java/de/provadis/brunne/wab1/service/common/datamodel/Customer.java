package de.provadis.brunne.wab1.service.common.datamodel;

import java.util.ArrayList;
import java.util.List;
import java.util.Objects;

public final class Customer {
    private final String id;
    private final String name;
    private final String email;
    private final String address;
    private final List<Order> orders;

    public Customer(String id, String name, String email, String address, List<Order> orders) {
        this.id = id;
        this.name = name;
        this.email = email;
        this.address = address;
        this.orders = orders;
    }

    public Customer(String id, String name, String email, String address) {
        this(id, name, email, address, new ArrayList<>());
    }

    public String getId() {
        return id;
    }

    public String getName() {
        return name;
    }

    public String getEmail() {
        return email;
    }

    public String getAddress() {
        return address;
    }

    public List<Order> getOrders() {
        return orders;
    }

    @Override
    public boolean equals(Object obj) {
        if (obj == this) return true;
        if (obj == null || obj.getClass() != this.getClass()) return false;
        var that = (Customer) obj;
        return Objects.equals(this.id, that.id) &&
                Objects.equals(this.name, that.name) &&
                Objects.equals(this.email, that.email) &&
                Objects.equals(this.address, that.address) &&
                Objects.equals(this.orders, that.orders);
    }

    @Override
    public int hashCode() {
        return Objects.hash(id, name, email, address, orders);
    }

    @Override
    public String toString() {
        return "Customer[" +
                "id=" + id + ", " +
                "name=" + name + ", " +
                "email=" + email + ", " +
                "address=" + address + ", " +
                "orders=" + orders + ']';
    }


}