<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>02_RAM_Bundle purchase</name>
   <tag></tag>
   <elementGuidId>c828cfd3-fcb6-4a89-a049-da2fb67e1929</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/xml; charset=utf-8</value>
   </httpHeaderProperties>
   <katalonVersion>7.8.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:cbs=&quot;http://oss.huawei.com/business/intf/webservice/cbs&quot; xmlns:msg=&quot;http://oss.huawei.com/business/intf/webservice/cbs/msg&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;cbs:WorkOrder>
         &lt;WorkOrderRequest>
            &lt;msg:WorkOrderType>29&lt;/msg:WorkOrderType>
            &lt;msg:SubscriberNo>${RAM_MSISDN}&lt;/msg:SubscriberNo>
            &lt;msg:operationCode>uvs&lt;/msg:operationCode>
            &lt;msg:password>******&lt;/msg:password>
            &lt;msg:AccessMode>8&lt;/msg:AccessMode>
            &lt;msg:SerialNo>1203110034946671142&lt;/msg:SerialNo>
            &lt;!--Optional:-->
            &lt;msg:ParaList>
               &lt;msg:ParaItem>
                  &lt;msg:ParaName>TradeTime&lt;/msg:ParaName>
                  &lt;msg:ParaValue>20171104093853&lt;/msg:ParaValue>
               &lt;/msg:ParaItem>
               &lt;msg:ParaItem>
                  &lt;msg:ParaName>SubCosid&lt;/msg:ParaName>
                  &lt;msg:ParaValue>500035&lt;/msg:ParaValue>
               &lt;/msg:ParaItem>
               &lt;msg:ParaItem>
                  &lt;msg:ParaName>SourceIP&lt;/msg:ParaName>
                  &lt;msg:ParaValue>10.245.109.81&lt;/msg:ParaValue>
               &lt;/msg:ParaItem>                                    
               &lt;msg:ParaItem>
                  &lt;msg:ParaName>AccessMode&lt;/msg:ParaName>
                  &lt;msg:ParaValue>2&lt;/msg:ParaValue>
               &lt;/msg:ParaItem>
            &lt;/msg:ParaList>
            &lt;msg:ProductOrderInfoList>
               &lt;msg:ProductOrderInfo>
                  &lt;msg:ProductID>${RAM_Bundle}&lt;/msg:ProductID>
                  &lt;msg:ProductOrderKey>999000002754891558&lt;/msg:ProductOrderKey>
                  &lt;msg:EffectiveDate>20171104093853&lt;/msg:EffectiveDate>
                  &lt;msg:ExpireDate>20180726105000&lt;/msg:ExpireDate>
                  &lt;msg:AutoType>1&lt;/msg:AutoType>
               &lt;/msg:ProductOrderInfo>
            &lt;/msg:ProductOrderInfoList>
            &lt;msg:OfferOrderInfoList>
               &lt;msg:OfferOrderInfo>
                  &lt;msg:OfferId>${RAM_Bundle}&lt;/msg:OfferId>
                  &lt;msg:OfferOrderKey>2746676979&lt;/msg:OfferOrderKey>
                  &lt;msg:EffectiveDate>20171104093853&lt;/msg:EffectiveDate>
                  &lt;msg:ExpireDate>20180725105000&lt;/msg:ExpireDate>
                  &lt;msg:AutoType>1&lt;/msg:AutoType>
               &lt;/msg:OfferOrderInfo>
            &lt;/msg:OfferOrderInfoList>
         &lt;/WorkOrderRequest>
      &lt;/cbs:WorkOrder>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>${Bundlepurchase_Url}</soapServiceEndpoint>
   <soapServiceFunction>Work</soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.Bundlepurchase_Url</defaultValue>
      <description></description>
      <id>56a0df6c-5ba3-41ca-8def-b902b236622c</id>
      <masked>false</masked>
      <name>Bundlepurchase_Url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.RAM_MSISDN</defaultValue>
      <description></description>
      <id>e020a813-def8-4d3f-aeec-5a2810c312a3</id>
      <masked>false</masked>
      <name>RAM_MSISDN</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.RAM_Bundle</defaultValue>
      <description></description>
      <id>e3d20f3b-f272-4d29-ae98-1b66d5001beb</id>
      <masked>false</masked>
      <name>RAM_Bundle</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress>file:/D:/shiva/Testing/014_Load%20balancer/BundlePurchase/bundlepurchase.wsdl.xml</wsdlAddress>
</WebServiceRequestEntity>
