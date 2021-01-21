<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>main menu</description>
   <name>Main menu</name>
   <tag></tag>
   <elementGuidId>273348b8-cdfd-4c32-ab65-8289d6ebeb0c</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;application/x-www-form-urlencoded&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: []
}</httpBodyContent>
   <httpBodyType>x-www-form-urlencoded</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/x-www-form-urlencoded</value>
   </httpHeaderProperties>
   <katalonVersion>7.8.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>http://10.200.178.76:8282/inject_mo?short_message=*128%23&amp;source_addr=243987654321&amp;destination_addr=98765432&amp;submit=Submit+Message&amp;service_type=&amp;source_addr_ton=1&amp;source_addr_npi=1&amp;dest_addr_ton=1&amp;dest_addr_npi=1&amp;esm_class=0&amp;protocol_ID=&amp;priority_flag=&amp;registered_delivery_flag=0&amp;data_coding=0&amp;user_message_reference=2022&amp;source_port=&amp;destination_port=&amp;sar_msg_ref_num=&amp;sar_total_segments=&amp;sar_segment_seqnum=&amp;user_response_code=&amp;privacy_indicator=&amp;payload_type=&amp;message_payload=&amp;callback_num=&amp;source_subaddress=&amp;dest_subaddress=&amp;language_indicator=&amp;tlv1_tag=1281&amp;tlv1_len=1&amp;tlv1_val=0001&amp;tlv2_tag=5376&amp;tlv2_len=30&amp;tlv2_val=1500&amp;tlv3_tag=5632&amp;tlv3_len=15&amp;tlv3_val=CF1D&amp;tlv4_tag=&amp;tlv4_len=&amp;tlv4_val=&amp;tlv5_tag=&amp;tlv5_len=&amp;tlv5_val=&amp;tlv6_tag=&amp;tlv6_len=&amp;tlv6_val=&amp;tlv7_tag=&amp;tlv7_len=&amp;tlv7_val=</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
