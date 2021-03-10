<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>03_RAM_Recharge</name>
   <tag></tag>
   <elementGuidId>27b595ee-9886-4a33-8b78-e486afb8e663</elementGuidId>
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
            &lt;msg:WorkOrderType>142&lt;/msg:WorkOrderType>
            &lt;msg:SubscriberNo>${RAM_MSISDN}&lt;/msg:SubscriberNo>
            &lt;msg:operationCode>uvs&lt;/msg:operationCode>
            &lt;msg:password>**a****&lt;/msg:password>
            &lt;msg:AccessMode>8&lt;/msg:AccessMode>
            &lt;msg:SerialNo>580101&lt;/msg:SerialNo>
            &lt;msg:ParaList>
               &lt;msg:ParaItem>
                  &lt;msg:ParaName>TansactionType&lt;/msg:ParaName>
                  &lt;msg:ParaValue>${RAM_TansactionType}&lt;/msg:ParaValue>
               &lt;/msg:ParaItem>
               &lt;msg:ParaItem>
                  &lt;msg:ParaName>CurrentPPSBalance&lt;/msg:ParaName>
                  &lt;msg:ParaValue>175000&lt;/msg:ParaValue>
               &lt;/msg:ParaItem>
               &lt;msg:ParaItem>
                  &lt;msg:ParaName>TradeTime&lt;/msg:ParaName>
                  &lt;msg:ParaValue>20200602101859&lt;/msg:ParaValue>
               &lt;/msg:ParaItem>
               &lt;msg:ParaItem>
                  &lt;msg:ParaName>SubCosID&lt;/msg:ParaName>
                  &lt;msg:ParaValue>40817&lt;/msg:ParaValue>
               &lt;/msg:ParaItem>
               &lt;msg:ParaItem>
                  &lt;msg:ParaName>RechargeAmount&lt;/msg:ParaName>
                  &lt;msg:ParaValue>${RAM_Recharge}&lt;/msg:ParaValue>
               &lt;/msg:ParaItem>
               &lt;msg:ParaItem>
                  &lt;msg:ParaName>LoanFlag&lt;/msg:ParaName>
                  &lt;msg:ParaValue>0&lt;/msg:ParaValue>
               &lt;/msg:ParaItem>
               &lt;msg:ParaItem>
                  &lt;msg:ParaName>AccessMode&lt;/msg:ParaName>
                  &lt;msg:ParaValue>0&lt;/msg:ParaValue>
               &lt;/msg:ParaItem>
            &lt;msg:ParaItem>
                  &lt;msg:ParaName>SourceIP&lt;/msg:ParaName>
                  &lt;msg:ParaValue>10.245.97.82&lt;/msg:ParaValue>
               &lt;/msg:ParaItem>              
               &lt;msg:ParaItem>
                  &lt;msg:ParaName>AccountType&lt;/msg:ParaName>
                  &lt;msg:ParaValue>0&lt;/msg:ParaValue>
               &lt;/msg:ParaItem>
               &lt;msg:ParaItem>
                  &lt;msg:ParaName>PreviousPOSBalance&lt;/msg:ParaName>
                  &lt;msg:ParaValue>0&lt;/msg:ParaValue>
               &lt;/msg:ParaItem>
               &lt;msg:ParaItem>
                  &lt;msg:ParaName>CurrentPOSBalance&lt;/msg:ParaName>
                  &lt;msg:ParaValue>0&lt;/msg:ParaValue>
               &lt;/msg:ParaItem>
               &lt;msg:ParaItem>
                  &lt;msg:ParaName>LoanAmount&lt;/msg:ParaName>
                  &lt;msg:ParaValue>0&lt;/msg:ParaValue>
               &lt;/msg:ParaItem>
            &lt;/msg:ParaList>
         &lt;/WorkOrderRequest>
      &lt;/cbs:WorkOrder>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope>
</soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>${Recharge_url}</soapServiceEndpoint>
   <soapServiceFunction>Work</soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.Recharge_Url</defaultValue>
      <description></description>
      <id>842636f7-cbce-408e-aa7b-cd7b18d374bc</id>
      <masked>false</masked>
      <name>Recharge_url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.RAM_MSISDN</defaultValue>
      <description></description>
      <id>f1f32c06-06b3-4a18-ac25-8ce197e6027c</id>
      <masked>false</masked>
      <name>RAM_MSISDN</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.RAM_Recharge</defaultValue>
      <description></description>
      <id>9b8bcf76-3f95-48e8-8875-97d2d019dec2</id>
      <masked>false</masked>
      <name>RAM_Recharge</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.RAM_TansactionType</defaultValue>
      <description></description>
      <id>84aea0dc-04bf-459c-9602-7a6bfe6632fe</id>
      <masked>false</masked>
      <name>RAM_TansactionType</name>
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
   <wsdlAddress>file:/C:/Softwares/76/Recharge/recharge.wsdl.xml</wsdlAddress>
</WebServiceRequestEntity>
