package de.provadis.brunne.wab1.server.common.datamodel;

import java.time.OffsetDateTime;
import java.util.List;

public record Order(String id,
                    OffsetDateTime orderedOn,
                    List<Product> products,
                    OrderStatus status) {

    public Order(String id,
                 List<Product> products) {
        this(id, OffsetDateTime.now(), products, OrderStatus.PENDING);
    }
}
